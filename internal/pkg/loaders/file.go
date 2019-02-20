package loaders

import (
	"encoding/json"
	"os"

	"github.com/SierraSoftworks/bender/pkg/models"

	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/pkg/errors"
)

type fileLoader struct {
	Filename string
}

func NewFileLoader(filename string) Loader {
	return &fileLoader{filename}
}

func (f *fileLoader) Load() ([]*models.Quote, error) {
	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"file":   f.Filename,
		"loader": "file",
	}).WithCategory("loaders").WithMessage("Loading quotes file")

	file, err := os.Open(f.Filename)
	if err != nil {
		return nil, errors.Wrap(err, "failed to open quotes file")
	}

	defer file.Close()

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"file":   f.Filename,
		"loader": "file",
	}).WithCategory("loaders").WithMessage("Parsing quotes file")

	quotes := []*models.Quote{}
	if err := json.NewDecoder(file).Decode(&quotes); err != nil {
		return nil, errors.Wrap(err, "failed to parse quotes file")
	}

	return quotes, nil
}

func (f *fileLoader) String() string {
	return "file://" + f.Filename
}
