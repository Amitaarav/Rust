interface Shape{
    area: () => number;
    perimeter: () => number;
}

class Rect implements Shape{
    constructor(){

    }

    area(){
        return 10;
    }

    perimeter(){
        return 10;
    }
}

// 
function getArea(s: Shape){
    
}