using Bender.Models;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;
using Xunit;

namespace Bender.Tests.Models
{
    public class HealthTests
    {
        [Fact]
        public void TestRepresentVersion1()
        {
            var representer = new Health.Version1.Representer();

            var model = new Health()
            {
                Started = DateTime.UtcNow
            };

            var view = representer.ViewFromModel(model);
            Assert.NotNull(view);
            Assert.Equal(view.Started, model.Started);

            var roundTrip = representer.ModelFromView(view);
            Assert.Equal(model.Started, roundTrip.Started);
        }

        [Fact]
        public void TestRenderVersion1()
        {
            var representer = new Health.Version1.Representer();

            var model = new Health()
            {
                Started = DateTime.UtcNow
            };

            var view = representer.ViewFromModel(model);

            var serialized = JsonConvert.SerializeObject(view);
            Assert.Equal(JsonConvert.SerializeObject(new
            {
                started = model.Started
            }), serialized);
        }
    }
}
