package loaders

import (
	"net/url"

	"github.com/SierraSoftworks/bender/pkg/models"
)

type Loader interface {
	Load() ([]*models.Quote, error)
	String() string
}

func New(path string) Loader {
	u, err := url.Parse(path)

	if err != nil {
		return NewFileLoader(path)
	}

	switch u.Scheme {
	case "gs":
		return NewGCSLoader(u.Host, u.Path[1:])
	case "http":
		fallthrough
	case "https":
		return NewHttpLoader(path)
	case "file":
		return NewFileLoader(u.Path)
	case "":
		return NewFileLoader(path)
	default:
		return nil
	}
}
