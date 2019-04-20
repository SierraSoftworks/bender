using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Bender.Config;
using Bender.Formatters;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.HttpsPolicy;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace Bender
{
    public class Startup
    {
        public Startup(IConfiguration configuration) => Configuration = configuration;

        public IConfiguration Configuration { get; }

        // This method gets called by the runtime. Use this method to add services to the container.
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddModelRepresenter<Health, Health.Version1, Health.Version1.Representer>();
            services.AddSingleton<HealthStore>();

            services.AddModelRepresenter<Quote, Quote.Version1, Quote.Version1.Representer>();
            services.Configure<FileQuoteStoreConfig>(Configuration);

            switch (Configuration.GetValue<string>("QuoteStore"))
            {
                case "BlobStorage":
                    services.AddSingleton<IQuoteStore, BlobQuoteStore<Quote.Version1>>();
                    break;
                case "File":
                    services.AddSingleton<IQuoteStore, FileQuoteStore<Quote.Version1>>();
                    break;
                case "Memory":
                default:
                    services.AddSingleton<IQuoteStore, MemoryQuoteStore>();
                    break;

            }

            services.AddApiVersioning(opts => opts.UseApiBehavior = true);

            services.AddResponseCompression();

            services.AddCors(options =>
            {
                options.AddDefaultPolicy(builder =>
                {
                    builder.AllowAnyOrigin();
                });
            });

            services.AddMvc()
                .AddNewtonsoftJson()
                .AddMvcOptions(options =>
                {
                    options.RespectBrowserAcceptHeader = true;
                    options.OutputFormatters.Add(new QuoteTextFormatter());
                    options.OutputFormatters.Add(new QuoteHtmlFormatter());
                })
                .SetCompatibilityVersion(CompatibilityVersion.Version_3_0);
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }
            else
            {
                // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
                app.UseHsts();
            }

            app.UseHttpsRedirection();
            app.UseCors(policy => policy.AllowAnyOrigin().WithMethods("GET"));

            app.UseCors();
            app.Use((context, next) =>
            {
                context.Items["__CorsMiddlewareInvoked"] = true;
                return next();
            });

            app.UseRouting()
                .UseResponseCompression()
                .UseEndpoints(routes => routes.MapControllers());

        }
    }
}
