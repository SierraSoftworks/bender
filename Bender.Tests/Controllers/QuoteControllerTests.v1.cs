using Bender.Models;
using Microsoft.AspNetCore.Mvc.Testing;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace Bender.Tests.Controllers
{
    public class QuoteControllerV1Tests
        : QuoteControllerTests<Quote.Version1, Quote.Version1.Representer>
    {
        public QuoteControllerV1Tests(BenderAppFactory factory) : base(factory)
        {
        }

        protected override string Version => "v1";
    }
}
