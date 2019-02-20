package api

import (
	"net/http"

	"github.com/SierraSoftworks/bender/internal/pkg/providers"

	"github.com/gorilla/mux"
	"github.com/rs/cors"
	log "github.com/sirupsen/logrus"
)

var registrars = []func(*API) error{}

func addRegistrar(r func(*API) error) {
	registrars = append(registrars, r)
}

type API struct {
	quotes *providers.QuoteProvider
	router *mux.Router
}

func NewAPI(quotes *providers.QuoteProvider) (*API, error) {
	api := &API{
		quotes: quotes,
		router: mux.NewRouter(),
	}

	for _, registrar := range registrars {
		if err := registrar(api); err != nil {
			return nil, err
		}
	}

	return api, nil
}

func (a *API) Quotes() *providers.QuoteProvider {
	return a.quotes
}

func (a *API) Router() *mux.Router {
	return a.router
}

func (a *API) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.Handle("/api/", a.router)
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(404)
		w.Write([]byte(`{"code": 404, "error": "Not Found", "message": "The method you attempted to make use of could not be found on our system."}`))
	})

	return cors.New(cors.Options{
		Debug: false,
	}).Handler(mux)
}

func (a *API) ListenAndServe(address string) error {
	log.WithField("address", address).Info("Starting server")
	return http.ListenAndServe(address, a.Handler())
}
