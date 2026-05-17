import init, {
    get_range,
    run_simulation
} from "../pkg/wasm.js";

async function main() {
    await init();

    const firstNumber =
        document.querySelector(".first_number");

    const secondNumber =
        document.querySelector(".second_number");

    const thirdNumber =
        document.querySelector(".third_number");

    const operationSelect =
        document.querySelector(".operation");

    const form =
        document.querySelector("form");

    const resultElement =
        document.querySelector(".result");

    function populateThirdNumber() {
        const dice = Number(firstNumber.value);

        const faces = Number(secondNumber.value);

        const range = get_range(dice, faces);

        thirdNumber.innerHTML = "";

        for (const value of range) {
            const option =
                document.createElement("option");

            option.value = value;
            option.textContent = value;

            thirdNumber.appendChild(option);
        }
    }

    form.addEventListener("submit", (event) => {
        event.preventDefault();

        const dice =
            Number(firstNumber.value);

        const faces =
            Number(secondNumber.value);

        const operation =
            operationSelect.value;

        const target =
            Number(thirdNumber.value);

        const result = run_simulation(
            10000000,
            dice,
            faces,
            operation,
            target
        );

        resultElement.textContent =
            `${result.toPrecision(2)}%`;
    });

    firstNumber.addEventListener(
        "change",
        populateThirdNumber
    );

    secondNumber.addEventListener(
        "change",
        populateThirdNumber
    );

    populateThirdNumber();
}

main();