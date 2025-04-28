fun fibonacci(n) {
    yehai a = 0;
    yehai b = 1;
    
    dikaho a;
    dikaho b;
    
    jabtak (n > 2) {
        yehai c = a + b;
        dikaho c;
        a = b;
        b = c;
        n = n - 1;
    }
}

fibonacci(10);
