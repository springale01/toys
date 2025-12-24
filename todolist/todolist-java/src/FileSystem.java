import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.util.List;

public class FileSystem {

    public void createFile(String fileName) {
        Path filePath = Paths.get(fileName);

        try {
            Files.writeString(
                    filePath,
                    "",
                    StandardOpenOption.CREATE
            );
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public List<String> readFile(Path file) {

        try {
            return Files.readAllLines(file, StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private void writeToFile(String content, Path file) {

        try {
            // does it overwrite?
            Files.writeString(file, content);
        } catch (
                IOException e
        ) {
            throw new RuntimeException(e);
        }
    }

    public void addOneLineToFile(String content, Path file) {
        if (content.lines().count() > 1 ) {
            content = content.split("\n", 2)[0]; //first line???
        }

        try {
            List<String> lines = readFile(file);
            lines.add(content);
            String stringed = String.join("\n", lines);
            writeToFile(stringed, file);

        } catch (RuntimeException e) {
            throw new RuntimeException(e);
        }
    }

    public void removeATask(Path file, int line_no) {
        List<String> readed = readFile(file);
        String deleted = readed.get(line_no);
        readed.remove(line_no);
        System.out.printf("Helper Spriggy: Deleted (%s) Successfully!" , deleted);

    }
}
