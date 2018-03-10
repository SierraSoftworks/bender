package api

import (
	"github.com/SierraSoftworks/girder"
	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/gorilla/mux"
)

func SentryRequestLogger(c *girder.Context) error {
	route := mux.CurrentRoute(c.Request)

	cl := sentry.DefaultClient().With(
		sentry.Culprit(route.GetName()),
		sentry.Logger("api"),
		sentry.HTTPRequest(c.Request).WithHeaders(),
	)

	cl.Capture(
		sentry.Message("Received Request for Route: %s", route.GetName()),
		sentry.Level(sentry.Info),
	)

	return nil
}
