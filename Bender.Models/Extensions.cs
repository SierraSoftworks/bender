using Bender.Models;
using Microsoft.Extensions.DependencyInjection;

namespace Microsoft.Extensions.DependencyInjection
{
    public static class Extensions
    {
        public static IServiceCollection AddModelRepresenter<TModel, TView, TRepresenter>(this IServiceCollection services)
            where TView : IView<TModel>
            where TRepresenter : class, IRepresenter<TModel, TView> => services.AddSingleton<IRepresenter<TModel, TView>, TRepresenter>();
    }
}