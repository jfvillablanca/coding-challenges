function findOdd(A) {

    A.sort((i,j) => (+i < +j) ? -1 : (+i > +j) ? 1 : 0);

    let counter = 0;
    while(A.length > 0){
        console.log(`A.length: ${A.length}`);
        if (A.length === 1) return A[0];
        if (A[0] === A[1]){
            counter++;
            console.log(`counter: ${counter}`);
            A.shift();
        }
        else{
            if(++counter % 2 === 1){
                console.log(`found odd: ${A[0]} with count ${counter}`);
                return A[0];
            }
            else{
                console.log(`"${A[0]}" count is even`);
                counter = 0;
                A.shift();
            }
        }
    }
    return 0;
}
