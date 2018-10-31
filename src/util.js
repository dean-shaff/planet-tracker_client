function Util(){
    this.radToDegree = function(val){
        if (typeof val !== Number){
            val = parseFloat(val, 10)
        }
        return (180./Math.PI)*val
    }
}
const util = new Util()
export default util
