import init, {calculate_tax} from "/pkg/tax_webassembly.js"

async function run() {
    await init()

    function calculateTax() {
        console.log(0)
        const income = parseFloat(document.getElementById("income").value)
        const tax = calculate_tax(income)
        document.getElementById("result").innerText = `Tax: $${tax.toFixed(2)}`
        console.log(0)
    }

    document.getElementById("calculateButton").addEventListener("click", calculateTax)
}
run()