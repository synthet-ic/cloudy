/*!
<https://github.com/grpc/grpc-go/blob/master/internal/grpcsync/oncefunc.go>
*/

use std::sync::Once;

// OnceFunc returns a function wrapping f which ensures f is only executed
// once even if the returned function is executed multiple times.
pub fn OnceFunc(f: func()) fn() {
    var once Once
    return || {
        once.Do(f)
    }
}
