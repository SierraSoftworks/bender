package api

import (
	"time"

	"github.com/SierraSoftworks/bender/pkg/models"
	"github.com/SierraSoftworks/girder"
)

func init() {
	addRegistrar(func(a *API) error {
		a.Router().
			Methods("GET").
			Path("/api/v1/status").
			Name("Health").
			Handler(girder.NewHandler(a.getStatus))

		return nil
	})
}

var started = time.Now()

func (a *API) getStatus(c *girder.Context) (interface{}, error) {
	return &models.Health{
		Started: started,
	}, nil
}
