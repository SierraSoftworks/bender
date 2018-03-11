package main

import (
	"os"
	"strings"
	"time"

	"github.com/SierraSoftworks/bender/api"
	"github.com/SierraSoftworks/bender/models"
	sentry "github.com/SierraSoftworks/sentry-go"
	log "github.com/Sirupsen/logrus"
	"github.com/pkg/errors"
	"github.com/urfave/cli"
)

var version = "development"
var sentryDSN = ""

func main() {
	sentry.AddInternalPrefixes("github.com/SierraSoftworks/bender")
	sentry.AddDefaultOptions(
		sentry.Release(version),
		sentry.DSN(sentryDSN),
	)
	if envDSN := os.Getenv("SENTRY_DSN"); envDSN != "" {
		sentry.AddDefaultOptions(sentry.DSN(envDSN))
	}

	defer sentry.DefaultSendQueue().Shutdown(true)

	app := cli.NewApp()
	app.Name = "Bender"
	app.Usage = "Run your very own BaaS (Bender as a Service)"

	app.Author = "Benjamin Pannell"
	app.Email = "admin@sierrasoftworks.com"
	app.Copyright = "Sierra Softworks © 2018"
	app.Version = version

	app.Flags = []cli.Flag{
		cli.StringFlag{
			Name:  "quotes",
			Usage: "The file containing quotes to load",
			Value: "quotes.json",
		},
		cli.StringFlag{
			Name:  "address",
			Usage: "The address to expose the server on",
			Value: ":8080",
		},
		cli.StringFlag{
			Name:  "log-level",
			Usage: "DEBUG|INFO|WARN|ERROR",
			Value: "INFO",
		},
	}

	app.Before = func(c *cli.Context) error {
		log.WithFields(log.Fields{
			"log-level": c.GlobalString("log-level"),
		}).Info("Starting")

		logLevel := c.GlobalString("log-level")
		switch strings.ToUpper(logLevel) {
		case "DEBUG":
			log.SetLevel(log.DebugLevel)
		case "INFO":
			log.SetLevel(log.InfoLevel)
		case "WARN":
			log.SetLevel(log.WarnLevel)
		case "ERROR":
			log.SetLevel(log.ErrorLevel)
		default:
			log.SetLevel(log.InfoLevel)
		}

		return nil
	}

	app.Action = func(c *cli.Context) error {
		quotes := models.NewQuoteProvider()
		if err := quotes.Load(c.String("quotes")); err != nil {
			log.WithError(err).Error("Failed to load quotes file")
			return errors.Wrap(err, "failed to load quotes file")
		}

		api, err := api.NewAPI(quotes)
		if err != nil {
			log.WithError(err).Error("Failed to initialize the API")
			return errors.Wrap(err, "failed to initialize the API")
		}

		return errors.Wrap(api.ListenAndServe(c.String("address")), "failed to start server")
	}

	if err := app.Run(os.Args); err != nil {
		log.WithError(err).Error("Failed to run application")

		e := sentry.DefaultClient().Capture(
			sentry.ExceptionForError(err),
			sentry.Level(sentry.Fatal),
		)
		select {
		case err, ok := <-e.WaitChannel():
			if ok && err != nil {
				log.WithError(err).Warn("Failed to send error to Sentry")
			}
		case <-time.After(time.Second):
			log.Warn("Timed out sending error to Sentry")
		}

		os.Exit(1)
	}
}
