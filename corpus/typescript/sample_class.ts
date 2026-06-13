class Counter {
    private value: number = 0;

    increment(): void {
        this.value += 1;
    }

    get(): number {
        return this.value;
    }
}
