import java.util.ArrayList;
import java.util.Objects;

public class Bank {
    final private String bank_owner = "Spriggy";
    private ArrayList<Account> accounts;

    public Bank() {
        this.accounts = new ArrayList<Account>();
    }

    public Account create_account(String name, int initial_deposit) {
        //bounce the poor people first bwahahaha
        if (initial_deposit < 200) {
            System.out.printf("%d isn't enough to make a initial deposit Mr.%s", initial_deposit, name);
            return null;
        } else if (check_dupes(name)) {
            System.out.printf("The name %s already exists! Please pick another one", name);
        }
        Account new_account = new Account(name, initial_deposit);
        accounts.add(new_account);
        System.out.println("New account created!");
        return new_account;
    }
    public void withdraw(String name, int amount) {
        Account target = find_account(name);
        if (target == null) {
            System.out.println("No Account Found Under The Name: " + name);
            return;
        }
        target.withdraw(amount);
        //bouncer
        if (target.get_balance() < 200) {
            System.out.println("You have under 200 moolahs prepare to get removed :P");
            int index = find_account_index(name);
            if (index == 69420) {
                System.out.println("No Account Found Under THe Name: " + name);
                return;
            }
            //yes IK this is dramatic but it's cool
            this.accounts.remove(index);
        }
    }
    public void deposit(String name, int amount) {
        Account target = find_account(name);
        if (target == null) {
            System.out.println("No Account Found Under The Name: " + name);
            return;
        }
        target.deposit(amount);
    }
    //returns balances
    public void inquire_account(String name) {
        int where_at = find_account_index(name);
        if (where_at == 69420) {
            System.out.println("Didn't find any accoutns under the name: " + name);
            return;
        }
        int bal = this.accounts.get(where_at).get_balance();
        System.out.println("Your balance is: " + bal + "\nUnder the name: " + name);
    }
    //returns index to that account
    private int find_account_index(String name) {

        for (int i = 0; i < this.accounts.size(); i++) {
            //No idea what this is
            if (Objects.equals(name, this.accounts.get(i).get_name())) {
                return i;
            }
        }
        return 69420; //THERE IS NO ERROR PROPAGATION
    }
    private Account find_account(String name) {
        for (Account account : this.accounts) {
            if (account.get_name().equals(name)) {
                return account;
            }
        }
        return null;
    }
    private boolean check_dupes(String name) {
        for (Account account : accounts) {
            if (Objects.equals(account.get_name(), name)) {
                return true;
            }
        }
        return false;
    }

}
