package api

import (
	"time"

	"github.com/SierraSoftworks/girder"
	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/gorilla/mux"
	log "github.com/sirupsen/logrus"
)

func SentryRequestLogger(c *girder.Context) error {
	route := mux.CurrentRoute(c.Request)

	e := sentry.NewClient(
		sentry.Culprit(route.GetName()),
		sentry.Logger("api"),
		sentry.HTTPRequest(c.Request).WithHeaders(),
	).Capture(
		sentry.Message("Received Request for Route: %s", route.GetName()),
		sentry.Level(sentry.Info),
	)

	go func() {
		select {
		case err, ok := <-e.WaitChannel():
			if ok && err != nil {
				log.WithError(err).Warn("Failed to send event to Sentry")
			}
		case <-time.After(1 * time.Second):
			log.Warn("Timeout sending event to Sentry")
		}
	}()

	return nil
}
