package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
	"sync"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/dialog"
	"fyne.io/fyne/v2/theme"
	"fyne.io/fyne/v2/widget"
	"github.com/speps/go-hashids/v2"
)

const (
	batchSize = 1000 // 每批处理的字符数
	pageSize  = 5000 // 每页显示的字符数
)

type converterApp struct {
	inputBox         *widget.Entry
	outputBox        *widget.Entry
	pageInfoLabel    *widget.Label // 输出页码信息标签
	inputPageLabel   *widget.Label // 输入页码信息标签
	window           fyne.Window
	mutex            sync.Mutex      // 添加互斥锁保护并发操作
	currentPage      int             // 输出当前页码
	totalPages       int             // 输出总页数
	fullOutput       string          // 完整输出内容
	fullInput        string          // 完整输入内容
	inputCurrentPage int             // 输入当前页码
	inputTotalPages  int             // 输入总页数
	hashids          *hashids.HashID // 加解密工具
}

// 加密函数
func (app *converterApp) encrypt(text string) string {
	num, err := strconv.ParseInt(text, 10, 64)
	if err != nil {
		return text
	}
	encrypted, err := app.hashids.EncodeInt64([]int64{num})
	if err != nil {
		return text
	}
	return encrypted
}

// 解密函数
func (app *converterApp) decrypt(text string) string {
	decoded, err := app.hashids.DecodeInt64WithError(text)
	if err != nil || len(decoded) == 0 {
		return text
	}
	return strconv.FormatInt(decoded[0], 10)
}

// 批量加密
func (app *converterApp) encryptBatch(input string) string {
	var builder strings.Builder
	items := strings.FieldsFunc(input, func(r rune) bool {
		return r == '\n' || r == ','
	})

	delimiter := ","
	if strings.Count(input, "\n") > strings.Count(input, ",") {
		delimiter = "\n"
	}

	for i, item := range items {
		if i > 0 {
			builder.WriteString(delimiter)
		}
		item = strings.TrimSpace(item)
		builder.WriteString(app.encrypt(item))
	}
	return builder.String()
}

// 批量解密
func (app *converterApp) decryptBatch(input string) string {
	var builder strings.Builder
	items := strings.FieldsFunc(input, func(r rune) bool {
		return r == '\n' || r == ','
	})

	delimiter := ","
	if strings.Count(input, "\n") > strings.Count(input, ",") {
		delimiter = "\n"
	}

	for i, item := range items {
		if i > 0 {
			builder.WriteString(delimiter)
		}
		item = strings.TrimSpace(item)
		builder.WriteString(app.decrypt(item))
	}
	return builder.String()
}

// 添加缺失的函数
func convertFormat(input string) string {
	var builder strings.Builder
	items := strings.FieldsFunc(input, func(r rune) bool {
		return r == '\n' || r == ','
	})

	// 检测输入是否主要使用换行符
	usesNewlines := strings.Count(input, "\n") > strings.Count(input, ",")

	// 根据输入格式选择相反的输出格式
	delimiter := ","
	if !usesNewlines {
		delimiter = "\n"
	}

	for i, item := range items {
		if i > 0 {
			builder.WriteString(delimiter)
		}
		item = strings.TrimSpace(item)
		builder.WriteString(item)
	}
	return builder.String()
}

// 添加缺失的函数
func removeQuotes(text string) string {
	var builder strings.Builder
	items := strings.FieldsFunc(text, func(r rune) bool {
		return r == '\n' || r == ','
	})

	delimiter := ","
	if strings.Contains(text, "\n") {
		delimiter = "\n"
	}

	for i, item := range items {
		if i > 0 {
			builder.WriteString(delimiter)
		}
		item = strings.TrimSpace(item)
		if strings.HasPrefix(item, "'") && strings.HasSuffix(item, "'") {
			builder.WriteString(item[1 : len(item)-1])
		} else {
			builder.WriteString(item)
		}
	}
	return builder.String()
}

func newConverterApp(a fyne.App) *converterApp {
	// 初始化hashids
	hd := hashids.NewData()
	hd.Salt = "Yout_Salt"
	hd.MinLength = 12
	h, _ := hashids.NewWithData(hd)

	app := &converterApp{
		inputBox:         widget.NewMultiLineEntry(),
		outputBox:        widget.NewMultiLineEntry(),
		pageInfoLabel:    widget.NewLabel(""),
		inputPageLabel:   widget.NewLabel(""),
		window:           a.NewWindow("运营百宝箱"),
		currentPage:      1,
		inputCurrentPage: 1,
		hashids:          h,
	}

	app.inputBox.PlaceHolder = "请输入内容（支持换行或逗号分隔，如果数据量过大，请使用一键粘贴功能）"
	app.outputBox.PlaceHolder = "输出结果将在这里显示"

	app.inputBox.Wrapping = fyne.TextWrapWord
	app.outputBox.Wrapping = fyne.TextWrapWord

	inputScroll := container.NewScroll(app.inputBox)
	outputScroll := container.NewScroll(app.outputBox)

	inputScroll.SetMinSize(fyne.NewSize(50, 80))
	outputScroll.SetMinSize(fyne.NewSize(50, 80))

	clearButtons := container.NewGridWithColumns(2,
		widget.NewButton("清空输入", func() { app.clearText(app.inputBox) }),
		widget.NewButton("清空输出", func() { app.clearText(app.outputBox) }),
	)

	// 输入框控制按钮
	inputButtons := container.NewGridWithColumns(4,
		widget.NewButton("中文逗号替换为英文逗号", func() { app.replaceCommas() }),
		widget.NewButton("添加单引号", func() { app.addRemoveQuotes(app.inputBox, "add") }),
		widget.NewButton("去除引号", func() { app.addRemoveQuotes(app.inputBox, "remove") }),
		widget.NewButtonWithIcon("一键粘贴", theme.ContentPasteIcon(), func() { app.pasteInput() }),
	)

	// 加解密按钮
	cryptoButtons := container.NewGridWithColumns(2,
		widget.NewButton("加密", func() {
			app.mutex.Lock()
			defer app.mutex.Unlock()
			inputText := app.fullInput
			if app.fullInput == "" {
				inputText = app.inputBox.Text
			}
			app.fullOutput = app.encryptBatch(inputText)
			app.currentPage = 1
			app.totalPages = int(math.Ceil(float64(len([]rune(app.fullOutput))) / float64(pageSize)))
			app.showCurrentPage()
		}),
		widget.NewButton("解密", func() {
			app.mutex.Lock()
			defer app.mutex.Unlock()
			inputText := app.fullInput
			if app.fullInput == "" {
				inputText = app.inputBox.Text
			}
			app.fullOutput = app.decryptBatch(inputText)
			app.currentPage = 1
			app.totalPages = int(math.Ceil(float64(len([]rune(app.fullOutput))) / float64(pageSize)))
			app.showCurrentPage()
		}),
	)

	// 输入框分页按钮
	inputPageButtons := container.NewGridWithColumns(4,
		widget.NewButton("首页", func() { app.goToInputPage(1) }),
		widget.NewButton("上一页", func() { app.goToInputPage(app.inputCurrentPage - 1) }),
		widget.NewButton("下一页", func() { app.goToInputPage(app.inputCurrentPage + 1) }),
		widget.NewButton("末页", func() { app.goToInputPage(app.inputTotalPages) }),
	)

	outputButtons := container.NewGridWithColumns(3,
		widget.NewButton("添加单引号", func() { app.addRemoveQuotes(app.outputBox, "add") }),
		widget.NewButton("去除引号", func() { app.addRemoveQuotes(app.outputBox, "remove") }),
		widget.NewButtonWithIcon("一键复制", theme.ContentCopyIcon(), func() { app.copyOutput() }),
	)

	// 输出框分页按钮
	pageButtons := container.NewGridWithColumns(4,
		widget.NewButton("首页", func() { app.goToPage(1) }),
		widget.NewButton("上一页", func() { app.goToPage(app.currentPage - 1) }),
		widget.NewButton("下一页", func() { app.goToPage(app.currentPage + 1) }),
		widget.NewButton("末页", func() { app.goToPage(app.totalPages) }),
	)

	// 创建页码信息容器，使页码信息居中显示
	inputPageContainer := container.NewCenter(app.inputPageLabel)
	pageInfoContainer := container.NewCenter(app.pageInfoLabel)

	mainLayout := container.NewVBox(
		widget.NewLabelWithStyle("输入内容", fyne.TextAlignCenter, fyne.TextStyle{Bold: true}),
		inputScroll,
		inputPageContainer,
		inputPageButtons,
		inputButtons,
		cryptoButtons,
		widget.NewButtonWithIcon("转换格式", theme.ConfirmIcon(), func() { app.updateOutput() }),
		widget.NewSeparator(),
		widget.NewLabelWithStyle("输出内容", fyne.TextAlignCenter, fyne.TextStyle{Bold: true}),
		outputScroll,
		pageInfoContainer,
		pageButtons,
		outputButtons,
		clearButtons,
	)

	app.window.SetContent(container.NewPadded(mainLayout))
	app.window.Resize(fyne.NewSize(80, 80))
	return app
}

func (app *converterApp) clearText(box *widget.Entry) {
	app.mutex.Lock()
	defer app.mutex.Unlock()
	box.SetText("")
	if box == app.outputBox {
		app.fullOutput = ""
		app.currentPage = 1
		app.totalPages = 0
		app.pageInfoLabel.SetText("")
	} else {
		app.fullInput = ""
		app.inputCurrentPage = 1
		app.inputTotalPages = 0
		app.inputPageLabel.SetText("")
	}
}

func (app *converterApp) pasteInput() {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	clipText := app.window.Clipboard().Content()
	if clipText == "" {
		return
	}

	app.fullInput = clipText
	app.inputCurrentPage = 1
	app.inputTotalPages = int(math.Ceil(float64(len([]rune(clipText))) / float64(pageSize)))
	app.showInputPage()
}

func (app *converterApp) showInputPage() {
	if app.fullInput == "" {
		app.inputBox.SetText("")
		app.inputPageLabel.SetText("")
		return
	}

	runes := []rune(app.fullInput)
	start := (app.inputCurrentPage - 1) * pageSize
	end := app.inputCurrentPage * pageSize
	if end > len(runes) {
		end = len(runes)
	}

	pageContent := string(runes[start:end])
	app.inputBox.SetText(pageContent)
	if app.inputTotalPages > 1 {
		app.inputPageLabel.SetText(fmt.Sprintf("第 %d/%d 页", app.inputCurrentPage, app.inputTotalPages))
	} else {
		app.inputPageLabel.SetText("")
	}
}

func (app *converterApp) goToInputPage(page int) {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	if page < 1 || page > app.inputTotalPages {
		return
	}
	app.inputCurrentPage = page
	app.showInputPage()
}

func (app *converterApp) updateOutput() {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	inputText := app.fullInput
	if app.fullInput == "" {
		inputText = app.inputBox.Text
	}
	app.fullOutput = convertFormat(inputText)
	app.currentPage = 1
	app.totalPages = int(math.Ceil(float64(len([]rune(app.fullOutput))) / float64(pageSize)))
	app.showCurrentPage()
}

func (app *converterApp) showCurrentPage() {
	if app.fullOutput == "" {
		app.outputBox.SetText("")
		app.pageInfoLabel.SetText("")
		return
	}

	runes := []rune(app.fullOutput)
	start := (app.currentPage - 1) * pageSize
	end := app.currentPage * pageSize
	if end > len(runes) {
		end = len(runes)
	}

	pageContent := string(runes[start:end])
	app.outputBox.SetText(pageContent)
	if app.totalPages > 1 {
		app.pageInfoLabel.SetText(fmt.Sprintf("第 %d/%d 页", app.currentPage, app.totalPages))
	} else {
		app.pageInfoLabel.SetText("")
	}
}

func (app *converterApp) goToPage(page int) {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	if page < 1 || page > app.totalPages {
		return
	}
	app.currentPage = page
	app.showCurrentPage()
}

func (app *converterApp) replaceCommas() {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	var builder strings.Builder
	inputText := app.fullInput
	if app.fullInput == "" {
		inputText = app.inputBox.Text
	}
	builder.Grow(len(inputText))

	runes := []rune(inputText)
	for i := 0; i < len(runes); i += batchSize {
		end := i + batchSize
		if end > len(runes) {
			end = len(runes)
		}

		for _, ch := range runes[i:end] {
			if ch == '，' {
				builder.WriteRune(',')
			} else {
				builder.WriteRune(ch)
			}
		}
	}

	app.fullInput = builder.String()
	app.inputCurrentPage = 1
	app.inputTotalPages = int(math.Ceil(float64(len([]rune(app.fullInput))) / float64(pageSize)))
	app.showInputPage()
}

func (app *converterApp) addRemoveQuotes(box *widget.Entry, action string) {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	var text string
	if box == app.outputBox {
		text = app.fullOutput
	} else {
		text = app.fullInput
		if app.fullInput == "" {
			text = app.inputBox.Text
		}
	}

	if text == "" {
		return
	}

	var builder strings.Builder
	builder.Grow(len(text) * 2)

	if action == "add" {
		items := strings.FieldsFunc(text, func(r rune) bool {
			return r == '\n' || r == ','
		})

		delimiter := ","
		if strings.Contains(text, "\n") {
			delimiter = "\n"
		}

		for i := 0; i < len(items); i += batchSize {
			end := i + batchSize
			if end > len(items) {
				end = len(items)
			}

			for j, item := range items[i:end] {
				if i+j > 0 {
					builder.WriteString(delimiter)
				}
				item = strings.TrimSpace(item)
				if !(strings.HasPrefix(item, "'") && strings.HasSuffix(item, "'")) {
					builder.WriteString("'")
					builder.WriteString(item)
					builder.WriteString("'")
				} else {
					builder.WriteString(item)
				}
			}
		}
	} else {
		builder.WriteString(removeQuotes(text))
	}

	result := builder.String()
	if box == app.outputBox {
		app.fullOutput = result
		app.currentPage = 1
		app.totalPages = int(math.Ceil(float64(len([]rune(result))) / float64(pageSize)))
		app.showCurrentPage()
	} else {
		app.fullInput = result
		app.inputCurrentPage = 1
		app.inputTotalPages = int(math.Ceil(float64(len([]rune(result))) / float64(pageSize)))
		app.showInputPage()
	}
}

func (app *converterApp) copyOutput() {
	app.mutex.Lock()
	defer app.mutex.Unlock()

	app.window.Clipboard().SetContent(app.fullOutput)
	dialog.ShowInformation("成功", "已成功复制到剪贴板！", app.window)
}

func main() {
	a := app.New()
	converter := newConverterApp(a)
	converter.window.ShowAndRun()
}
