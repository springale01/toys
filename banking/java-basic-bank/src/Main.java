import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        //THIS IS A JOKE I SWEAR
        System.out.println("Welcome to Spriggy Bank, If your poor please turn left into BOFA");
        System.out.println("""
                What would you like to do?\
                
                Open A Account\
                
                Deposit into a Account\
                
                Inquire about your Account""");
        boolean closed = false;
        Bank spriggy_bank = new Bank();
        int frustration_count = 0;
        while (!closed) {
            String input = scanner.nextLine().trim().toLowerCase();
            switch (input) {
                case "open" -> {
                    System.out.println("Enter your name: ");
                    String name = scanner.nextLine().trim();
                    System.out.println("Initial Deposit: ");
                    int deposit = scanner.nextInt();
                    scanner.nextLine();
                    //account creating arc
                    Account new_account = spriggy_bank.create_account(name, deposit);
                    if (new_account == null) {
                        System.out.println("Account Creation Failed, Reason: Too Broke");
                        continue;
                    }
                    System.out.println("Your new account is created!");
                }
                case "deposit" -> {
                    System.out.println("Enter your name");
                    String name = scanner.nextLine().trim();
                    System.out.println("How much to deposit?");
                    int deposit = scanner.nextInt();
                    scanner.nextLine();
                    spriggy_bank.deposit(name, deposit);
                }
                case "check balance", "check" -> {
                    System.out.println("Enter your name");
                    String name = scanner.nextLine().trim();
                    spriggy_bank.inquire_account(name);
                }
                default -> {
                    System.out.println("The Clerk Spriggy Couldn't Understand You!");
                    frustration_count += 1;
                }
            }
            if (frustration_count > 5) {
                closed = true;
                System.out.println("Spriggy has Closed the bank...");
            }
        }
        scanner.close();
    }

}
