using System;

namespace Bender.Models
{
    public interface IRepresenter<TModel, TView>
        where TView : IView<TModel>
    {
        TView ViewFromModel(TModel model);

        TModel ModelFromView(TView view);
    }
}
