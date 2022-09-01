package result

import (
	"C"
	"encoding/base64"
)
import "fmt"

func markError(data []byte) []byte {
	return append([]byte{0}, data...)
}

func markOk(data []byte) []byte {
	return append([]byte{1}, data...)
}

func EncodeResultFromError(err error) *C.char {
	marked := markError([]byte(err.Error()))
	res := base64.StdEncoding.EncodeToString(marked)
	fmt.Printf("err: %s\n", res)
	return C.CString(res)
}

func EncodeResultFromOk(data []byte) *C.char {
	marked := markOk(data)
	res := base64.StdEncoding.EncodeToString(marked)
	fmt.Printf("ok: %s\n", res)
	return C.CString(res)
}
