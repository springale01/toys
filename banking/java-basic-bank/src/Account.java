public class Account {
    private String owner;
    private int balance;

    //I think this is the New function?
    public Account(String owner, int starting_balance) {
        this.owner = owner;
        this.balance = starting_balance;
    }

    public void deposit(int deposit_amount) {
        this.balance += deposit_amount;
        System.out.printf("Deposit Completed, current balance %d", this.balance);
    }

    public void withdraw(int withdraw_amount) {
        this.balance -= withdraw_amount;
        System.out.printf("Account Balance Updated:\n, %d remaining, %d withdrew", this.balance, withdraw_amount);
    }

    public void check_balance() {
        System.out.printf("%s current has %d in their account!", this.owner, this.balance);
    }

    public void change_name(String new_name) {
        System.out.printf("Name Change Successful, form %s to %s.", this.owner, new_name);
        this.owner = new_name; // Hopefully this doesn't die
    }

    public String get_name() {
        return this.owner;
    }
    public int get_balance() {
        return this.balance;
    }
}
