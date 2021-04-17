module.exports = function(input) {
    let floor = 0;
    let basement = null;

    //Note idx is the char index (e.g. 0 or 15) but as a string (e.g. "0" or "15")
    for (const idx in input) {
        switch (input[idx]) {
            case '(':
                floor++;
                break;
            case ')':
                floor--;
                if (floor < 0 && basement == null) {
                    //parseInt needed as per note above
                    basement = parseInt(idx) + 1;
                }
                break;
        }
    }

    return [floor.toString(), basement.toString()];
}