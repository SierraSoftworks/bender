package main

import (
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/SierraSoftworks/bender/internal/pkg/api"
	"github.com/SierraSoftworks/bender/internal/pkg/providers"
	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
	"github.com/urfave/cli"
)

var version = "development"
var sentryDSN = "https://0f9ec16cd0e2473bb994e7108d951b86@sentry.io/1362607"

func main() {
	sentryQueue := sentry.NewSequentialSendQueue(10)
	defer sentryQueue.Shutdown(true)

	sentry.AddInternalPrefixes("github.com/SierraSoftworks/bender")
	sentry.AddDefaultOptions(
		sentry.Release(version),
		sentry.DSN(sentryDSN),
		sentry.UseSendQueue(sentryQueue),
	)
	if envDSN := os.Getenv("SENTRY_DSN"); envDSN != "" {
		sentry.AddDefaultOptions(sentry.DSN(envDSN))
	}

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
		cli.IntFlag{
			Name:   "port",
			Usage:  "The port to expose the server on",
			Value:  8080,
			EnvVar: "PORT",
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
		quotes := providers.NewQuoteProvider()
		if err := quotes.Load(c.String("quotes")); err != nil {
			log.WithError(err).Error("Failed to load quotes file")
			return errors.Wrap(err, "failed to load quotes file")
		}

		api, err := api.NewAPI(quotes)
		if err != nil {
			log.WithError(err).Error("Failed to initialize the API")
			return errors.Wrap(err, "failed to initialize the API")
		}

		return errors.Wrap(api.ListenAndServe(fmt.Sprintf(":%d", c.Int("port"))), "failed to start server")
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
