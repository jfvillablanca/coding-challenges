function spinWords (string){
    const words = string.split(' ');

    words.forEach((word,i) =>
        {
            if(words[i].length >= 5){
                words[i] = reverseString(word);
            }
        });
    return words.join(' ');
}

const reverseString = function(string) {
        let stringArray = string.split('');
        let reverseArray = [];
    for(let i = 0; i < stringArray.length; i++){
                reverseArray.push(stringArray[stringArray.length - 1 - i]);

    }
        return reverseArray.toString().replace(/,/g,'');

};
