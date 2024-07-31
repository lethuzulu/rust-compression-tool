let name = 'lethukuthula'

let iterator = name[Symbol.iterator]()

let holder = {}

for (i of iterator) {
    //if i is not in holder, put it in holder, and set value to 1
    if (i in holder) {
        holder[i] = holder[i] + 1
    } else {
        holder[i] = 1
    }
}

console.log('holder  ', holder  )
