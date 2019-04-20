using Bender.Models;
using Microsoft.AspNetCore.Http;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Bender.Formatters
{
    public class QuoteHtmlFormatter : ModelFormatter<Quote.Version1>
    {
        public QuoteHtmlFormatter()
            : base("text/html")
        {

        }

        private readonly string template = @"
            <html>
                <head>
                    <style>
                        body {{
                            font-family: Sans-serif;
                        }}

                        figure {{
                            margin: 20px;
                        }}

                        blockquote {{
                            margin-left: 1em;
                        }}

                        figcaption {{
                            margin-left: 2em;
                            font-size: 0.8em;
                            font-weight: bold;
                        }}

                        figcaption::before {{
                            display: inline;
                            content: ""–"";
                            padding-right: 0.5em;
                        }}
                    </style>
                    <title>Bender as a Service</title>
                </head>
                <body>
                    <figure>
                        <blockquote>{0}</blockquote>
                        <figcaption>{1}</figcaption>
                    </figure>
                </body>
            </html>";

        public override async Task RenderModelAsync(Quote.Version1 model, HttpResponse response, Encoding selectedEncoding) => await response.WriteAsync(string.Format(this.template, model.Quote, model.Who), selectedEncoding);
    }
}
