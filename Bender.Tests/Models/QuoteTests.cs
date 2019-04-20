using Bender.Models;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;
using Xunit;

namespace Bender.Tests.Models
{
    public class QuoteTests
    {
        [Fact]
        public void TestRepresentVersion1()
        {
            var representer = new Quote.Version1.Representer();

            var model = new Quote()
            {
                Who = "Bender",
                Text = "Bite my shiny metal ass!"
            };

            var view = representer.ViewFromModel(model);
            Assert.NotNull(view);
            Assert.Equal(view.Quote, model.Text);
            Assert.Equal(view.Who, model.Who);

            var roundTrip = representer.ModelFromView(view);
            Assert.Equal(model.Text, roundTrip.Text);
            Assert.Equal(model.Who, roundTrip.Who);
        }

        [Fact]
        public void TestRenderVersion1()
        {
            var representer = new Quote.Version1.Representer();

            var model = new Quote()
            {
                Who = "Bender",
                Text = "Bite my shiny metal ass!"
            };

            var view = representer.ViewFromModel(model);

            var serialized = JsonConvert.SerializeObject(view);
            Assert.Equal(JsonConvert.SerializeObject(new {
                quote = "Bite my shiny metal ass!",
                who = "Bender",
            }), serialized);
        }
    }
}
