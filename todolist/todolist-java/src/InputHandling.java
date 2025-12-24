import java.util.Scanner;

public class InputHandling {
    // Callers call the function and get grab the inputted input form the current input?
    public String currentInput;
    public int currentInputInt;
    // Since it's a todo app I don't think I should include ints
    public void grabInput(String message) {
        //Not sure pubbing the scanner here(SKULL)
        Scanner scanner = new Scanner(System.in);
        System.out.println(message);
        currentInput = scanner.nextLine();

    }

    public void grabInputInt(String message) {
        Scanner scanner = new Scanner(System.in);
        System.out.println(message);
        currentInputInt = scanner.nextInt();

    }
}
