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
    public class HealthControllerV1Tests
        : HealthControllerTests<Health.Version1, Health.Version1.Representer>
    {
        public HealthControllerV1Tests(WebApplicationFactory<Startup> factory) : base(factory)
        {
        }

        protected override string Version => "v1";
    }
}
