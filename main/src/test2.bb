class List {
    fun init() {
        this.items = [];
    }

    fun add(item) {
        this.items.push(item);
    }

    fun get(index) {
        return this.items[index];
    }

    fun set(index, value) {
        this.items[index] = value;
    }

    fun size() {
        return this.items.length;
    }
}

fun swap(list, i, j) {
    yehai temp = list.get(i);
    list.set(i, list.get(j));
    list.set(j, temp);
}

fun bubbleSort(list) {
    yehai n = list.size();
    jabtak (n > 1) {
        yehai i = 0;
        jabtak (i < n - 1) {
            if (list.get(i) > list.get(i + 1)) {
                swap(list, i, i + 1);
            }
            i = i + 1;
        }
        n = n - 1;
    }
}

// Ab hum list banate hain
yehai numbers = List();
numbers.add(5);
numbers.add(2);
numbers.add(9);
numbers.add(1);
numbers.add(5);
numbers.add(6);

bubbleSort(numbers);

// Sorted list print karte hain
yehai i = 0;
jabtak (i < numbers.size()) {
    dikaho numbers.get(i);
    i = i + 1;
}
