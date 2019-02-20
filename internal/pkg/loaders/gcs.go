package loaders

import (
	"context"
	"encoding/json"

	"cloud.google.com/go/storage"
	"github.com/SierraSoftworks/bender/pkg/models"
	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/pkg/errors"
)

type gcsLoader struct {
	Bucket   string
	Filename string
}

func NewGCSLoader(bucket, filename string) Loader {
	return &gcsLoader{
		Bucket:   bucket,
		Filename: filename,
	}
}

func (l *gcsLoader) Load() ([]*models.Quote, error) {
	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"bucket": l.Bucket,
		"file":   l.Filename,
		"loader": "gcs",
	}).WithCategory("loaders").WithMessage("Loading quotes file from Google Cloud Storage")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	client, err := storage.NewClient(ctx)
	if err != nil {
		return nil, errors.Wrap(err, "failed to access cloud storage account")
	}

	rc, err := client.Bucket(l.Bucket).Object(l.Filename).NewReader(ctx)
	if err != nil {
		return nil, errors.Wrap(err, "failed to access cloud storage object")
	}
	defer rc.Close()

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"bucket": l.Bucket,
		"file":   l.Filename,
		"loader": "gcs",
	}).WithCategory("loaders").WithMessage("Parsing quotes file from Google Cloud Storage")

	quotes := []*models.Quote{}
	if err := json.NewDecoder(rc).Decode(&quotes); err != nil {
		return nil, errors.Wrap(err, "failed to parse quotes file")
	}

	return quotes, nil
}

func (l *gcsLoader) String() string {
	return "gs://" + l.Bucket + "/" + l.Filename
}
