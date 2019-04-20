using Bender.Models;
using System.Net;
using System.Net.Http;
using System.Threading.Tasks;
using Xunit;

namespace Bender.Tests.Controllers
{
    public abstract class QuoteControllerTests<TView, TRepresenter>
        : IClassFixture<BenderAppFactory>
        where TView : IView<Quote>
        where TRepresenter : IRepresenter<Quote, TView>, new()
    {
        public QuoteControllerTests(BenderAppFactory factory)
        {
            this.Factory = factory;
            this.Representer = new TRepresenter();
        }

        protected abstract string Version { get; }

        BenderAppFactory Factory { get; }

        public IRepresenter<Quote, TView> Representer { get; }

        [Theory]
        [InlineData("/api/v1/quote", "https://blog.sierrasoftworks.com")]
        [InlineData("/api/v1/quote", "https://example.com")]
        [InlineData("/api/v1/quote/bender", "https://blog.sierrasoftworks.com")]
        [InlineData("/api/v1/quote/bender", "https://example.com")]
        public async Task TestCors(string endpoint, string origin)
        {
            var client = Factory.CreateClient();
            var request = new HttpRequestMessage(HttpMethod.Get, endpoint);
            request.Headers.Add("Origin", origin);
            var response = await client.SendAsync(request);
            Assert.Equal(HttpStatusCode.OK, response.StatusCode);
            Assert.Contains("*", response.Headers.GetValues("Access-Control-Allow-Origin"));
        }

        [Fact]
        public async Task TestGetQuoteNotFound()
        {
            await Factory.ClearQuotesAsync();

            var client = Factory.CreateClient();

            var response = await client.GetAsync($"/api/{Version}/quote");
            Assert.Equal(HttpStatusCode.NotFound, response.StatusCode);

            Assert.Equal("application/problem+json", response.Content.Headers.ContentType.MediaType);
        }

        [Fact]
        public async Task TestGetQuote()
        {
            await Factory.ClearQuotesAsync();
            await Factory.AddQuoteAsync(new Quote
            {
                Text = "Bite my shiny metal ass!",
                Who = "Bender"
            });

            var client = Factory.CreateClient();

            var response = await client.GetAsync($"/api/{Version}/quote");
            response.EnsureSuccessStatusCode();

            Assert.Equal("application/json", response.Content.Headers.ContentType.MediaType);

            var view = await response.Content.ReadAsAsync<TView>();
            var model = Representer.ModelFromView(view);
            Assert.Equal("Bender", model.Who);
            Assert.Equal("Bite my shiny metal ass!", model.Text);
        }

        [Theory]
        [InlineData("text/plain", "Bite my shiny metal ass! – Bender")]
        [InlineData("text/html", "<blockquote>Bite my shiny metal ass!</blockquote>")]
        [InlineData("text/html", "<figcaption>Bender</figcaption>")]
        public async Task TestGetQuoteFormats(string contentType, string containsText)
        {
            await Factory.ClearQuotesAsync();
            await Factory.AddQuoteAsync(new Quote
            {
                Text = "Bite my shiny metal ass!",
                Who = "Bender"
            });

            var client = Factory.CreateClient();

            client.DefaultRequestHeaders.Accept.ParseAdd(contentType);

            var response = await client.GetAsync($"/api/{Version}/quote");
            response.EnsureSuccessStatusCode();

            Assert.Equal(contentType, response.Content.Headers.ContentType.MediaType);
            Assert.Contains(containsText, await response.Content.ReadAsStringAsync());
        }

        [Fact]
        public async Task TestGetQuoteBy()
        {
            await Factory.ClearQuotesAsync();
            await Factory.AddQuoteAsync(new Quote
            {
                Text = "Bite my shiny metal ass!",
                Who = "Bender"
            });

            var client = Factory.CreateClient();

            var response = await client.GetAsync($"/api/{Version}/quote/bender");
            response.EnsureSuccessStatusCode();

            Assert.Equal("application/json", response.Content.Headers.ContentType.MediaType);

            var view = await response.Content.ReadAsAsync<TView>();
            var model = Representer.ModelFromView(view);
            Assert.Equal("Bender", model.Who);
            Assert.Equal("Bite my shiny metal ass!", model.Text);
        }

        [Theory]
        [InlineData("bender", HttpStatusCode.OK)]
        [InlineData("Bender", HttpStatusCode.OK)]
        [InlineData("ben", HttpStatusCode.NotFound)]
        public async Task TestGetQuoteByIsCaseInsensitive(string author, HttpStatusCode statusCode)
        {
            await Factory.ClearQuotesAsync();
            await Factory.AddQuoteAsync(new Quote
            {
                Text = "Bite my shiny metal ass!",
                Who = "Bender"
            });

            var client = Factory.CreateClient();

            var response = await client.GetAsync($"/api/{Version}/quote/{author}");
            Assert.Equal(statusCode, response.StatusCode);
        }
    }
}
