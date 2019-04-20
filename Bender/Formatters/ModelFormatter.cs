using Bender.Models;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc.Formatters;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Bender.Formatters
{
    public abstract class ModelFormatter<TView> : TextOutputFormatter
        where TView : IView<Quote>
    {
        public ModelFormatter(params string[] contentType)
        {
            foreach(var type in contentType)
                SupportedMediaTypes.Add(type);

            SupportedEncodings.Add(Encoding.UTF8);
            SupportedEncodings.Add(Encoding.UTF32);
            SupportedEncodings.Add(Encoding.ASCII);
            SupportedEncodings.Add(Encoding.Unicode);
            SupportedEncodings.Add(Encoding.Default);
        }

        protected override bool CanWriteType(Type type) => type == typeof(TView);

        public override async Task WriteResponseBodyAsync(OutputFormatterWriteContext context, Encoding selectedEncoding) => await RenderModelAsync((TView)context.Object, context.HttpContext.Response, selectedEncoding);

        public abstract Task RenderModelAsync(TView model, HttpResponse response, Encoding selectedEncoding);
    }
}
