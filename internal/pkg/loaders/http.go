package loaders

import (
	"encoding/json"
	"net/http"

	"github.com/SierraSoftworks/bender/pkg/models"

	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/pkg/errors"
)

type httpLoader struct {
	URL string
}

func NewHttpLoader(url string) Loader {
	return &httpLoader{url}
}

func (f *httpLoader) Load() ([]*models.Quote, error) {
	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"url":    f.URL,
		"loader": "http",
	}).WithCategory("loaders").WithMessage("Loading quotes file")

	res, err := http.DefaultClient.Get(f.URL)
	if err != nil {
		return nil, errors.Wrap(err, "failed to request quotes file")
	}

	defer res.Body.Close()

	sentry.DefaultBreadcrumbs().NewHTTPRequest("GET", f.URL, res.StatusCode, res.Status)

	if res.StatusCode >= 400 {
		return nil, errors.Errorf("failed to request quotes file: http: got %s", res.Status)
	}

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"url": f.URL,
	}).WithCategory("models").WithMessage("Parsing quotes file")

	quotes := []*models.Quote{}
	if err := json.NewDecoder(res.Body).Decode(&quotes); err != nil {
		return nil, errors.Wrap(err, "failed to parse quotes file")
	}

	return quotes, nil
}

func (f *httpLoader) String() string {
	return f.URL
}
