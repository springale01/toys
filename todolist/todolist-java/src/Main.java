import java.nio.file.Path;

import java.util.List;

public class Main {
    public static void main(String[] args) {
        // booting up....
        spriggyTalks("Welcome to the TodoList app!");
        FileSystem fs = new FileSystem();
        fs.createFile("Joe.txt");
        Path buffer = Path.of("Joe.txt");
        //Loop
        boolean yeet = true;
        InputHandling inputsys = new InputHandling();
        while (yeet) {
            inputsys.grabInput("What would you like to do?\nCurrent Tasks\nAdd a new Task\nRemove a Task\nExit");
            String currentInput = inputsys.currentInput.toLowerCase();
            switch (currentInput) {
                case "tasks":
                    List<String> items = fs.readFile(buffer);
                    String joined = String.join("\n", items);

                    System.out.println(spriggyScribbles("\n" + joined));
                    break;

                case "add":
                    inputsys.grabInput(spriggyScribbles("What tasks would you like to add?"));
                    // I COULD make a function to better this but, I'm too lazy rn
                    String input = inputsys.currentInput.toLowerCase();
                    fs.addOneLineToFile(input, buffer);
                    spriggyTalks("Tasks added successfully!");
                    break;

                case "remove":
                    inputsys.grabInputInt(spriggyScribbles("What tasks you want to remove?"));
                    int inputInt = inputsys.currentInputInt;
                    fs.removeATask(buffer, inputInt);
                    spriggyTalks("Deleted Successfully!");
                    break;

                default:
                    spriggyTalks("Oki, bye bye!");
                    yeet = false;
            }


        }

    }

    private static void spriggyTalks(String content) {
        System.out.printf("Helper Spriggy: %s%n", content);
    }

    private static String spriggyScribbles(String content) {
        return "Helper Spriggy: " + content;
    }
}