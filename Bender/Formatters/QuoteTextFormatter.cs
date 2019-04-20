using Bender.Models;
using Microsoft.AspNetCore.Http;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Bender.Formatters
{
    public class QuoteTextFormatter : ModelFormatter<Quote.Version1>
    {
        public QuoteTextFormatter()
            : base("text/plain")
        {

        }

        public override async Task RenderModelAsync(Quote.Version1 model, HttpResponse response, Encoding selectedEncoding) => await response.WriteAsync($"{model.Quote} – {model.Who}", selectedEncoding);
    }
}
