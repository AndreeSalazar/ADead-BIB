// 03_clases.js — JsDead-BIB Test #3
// Clases → Structs + methods (igual que C++)

class Point {
    x: int
    y: int

    constructor(x: int, y: int) {
        this.x = x;
        this.y = y;
    }

    distanceSquared(): int {
        return this.x * this.x + this.y * this.y;
    }

    print(): void {
        console.log(this.x);
        console.log(this.y);
    }
}

class Rectangle {
    width: int
    height: int

    constructor(w: int, h: int) {
        this.width = w;
        this.height = h;
    }

    area(): int {
        return this.width * this.height;
    }

    perimeter(): int {
        return 2 * (this.width + this.height);
    }
}

function main(): void {
    let p: Point = new Point(3, 4);
    p.print();
    let dist: int = p.distanceSquared();
    console.log(dist);

    let r: Rectangle = new Rectangle(5, 10);
    let a: int = r.area();
    console.log(a);
    let per: int = r.perimeter();
    console.log(per);
}

main();
