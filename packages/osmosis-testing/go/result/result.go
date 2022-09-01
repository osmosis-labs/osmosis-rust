package result

import (
	"C"
	"encoding/base64"
)

func markError(data []byte) []byte {
	return append([]byte{0}, data...)
}

func markOk(data []byte) []byte {
	return append([]byte{1}, data...)
}

func EncodeResultFromError(err error) string {
	marked := markError([]byte(err.Error()))
	return base64.StdEncoding.EncodeToString(marked)
}

func EncodeResultFromOk(data []byte) string {
	marked := markOk(data)
	return base64.StdEncoding.EncodeToString(marked)
}
