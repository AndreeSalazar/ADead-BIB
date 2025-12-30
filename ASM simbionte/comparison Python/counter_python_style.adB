// ADead-BIB con sintaxis Python-style
// Mismo resultado, sintaxis más familiar para usuarios de Python

class Counter:
    def run(self, limit):
        let i = 0
        while i < limit {
            i = i + 1
        }
        return i

fn main() {
    println(Counter::run(1000000000))
}
