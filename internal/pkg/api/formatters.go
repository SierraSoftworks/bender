package api

import (
	"fmt"
	"html"
	"io"

	"github.com/SierraSoftworks/bender/pkg/models"
)

type TextFormatter struct {
}

func (f *TextFormatter) Write(data interface{}, into io.Writer) error {
	s := fmt.Sprintf("%v", data)
	if quote, ok := data.(*models.Quote); ok {
		s = fmt.Sprintf(`"%s" – %s`, quote.Quote, quote.Who)
	}

	_, err := io.WriteString(into, s)
	return err
}

type HtmlFormatter struct {
}

func (f *HtmlFormatter) Write(data interface{}, into io.Writer) error {
	if quote, ok := data.(*models.Quote); ok {
		tmpl := `<html>
	<head>
		<style>
			body {
				font-family: Sans-serif;
			}

			figure {
				margin: 20px;
			}

			blockquote {
				margin-left: 1em;
			}

			figcaption {
				margin-left: 2em;
				font-size: 0.8em;
				font-weight: bold;
			}

			figcaption::before {
				display: inline;
				content: "–";
				padding-right: 0.5em;
			}
		</style>
	</head>
	<body>
		<figure>
			<blockquote>%s</blockquote>
			<figcaption>%s</figcaption>
		</figure>
	</body>
</html>`
		s := fmt.Sprintf(tmpl, html.EscapeString(quote.Quote), html.EscapeString(quote.Who))

		_, err := io.WriteString(into, s)
		return err
	}

	return fmt.Errorf("unknown datatype for representation as html")
}
