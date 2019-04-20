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
    public abstract class HealthControllerTests<TView, TRepresenter>
		: IClassFixture<WebApplicationFactory<Startup>>
        where TView : IView<Health>
        where TRepresenter : IRepresenter<Health, TView>, new()
    {
		public HealthControllerTests(WebApplicationFactory<Startup> factory)
        {
            this.Factory = factory;
            this.Representer = new TRepresenter();
        }

        protected abstract string Version { get; }

		WebApplicationFactory<Startup> Factory { get; }

        public IRepresenter<Health, TView> Representer { get; }

        [Fact]
		public async Task TestGetHealth()
        {
            var client = Factory.CreateClient();

            var response = await client.GetAsync($"/api/{Version}/health");
            response.EnsureSuccessStatusCode();

            Assert.Equal("application/json", response.Content.Headers.ContentType.MediaType);

            var view = await response.Content.ReadAsAsync<TView>();
            var model = Representer.ModelFromView(view);
            Assert.Equal(DateTime.UtcNow, model.Started, TimeSpan.FromSeconds(1));
        }
    }
}
