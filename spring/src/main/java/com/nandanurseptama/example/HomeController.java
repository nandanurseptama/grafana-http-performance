package com.nandanurseptama.example;

import org.springframework.web.bind.annotation.RestController;

import java.util.HashMap;
import java.util.Map;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;

@RestController("/")
public class HomeController {

    @GetMapping("")
    public ResponseEntity<Map<String, Object>> hello(
            @RequestParam(defaultValue = "user", required = false) String name) {
        final Map<String, Object> data = new HashMap<String, Object>();
        data.put("data", "hello " + name);
        data.put("message", "OK");
        return ResponseEntity.ok().body(data);

    }

    @GetMapping("fibonacci")
    public ResponseEntity<Map<String, Object>> getFibonacci(@RequestParam Long n) {
        final Map<String, Object> data = new HashMap<String, Object>();
        try {
            if (n < 1) {
                data.put("data", null);
                data.put("message", "n must be start from 1");
                return ResponseEntity.ok().body(data);
            }
            final Long d = this.fibonacci(n);
            data.put("data", d);
            data.put("message", "OK");
            return ResponseEntity.ok().body(data);
        } catch (Exception e) {
            data.put("data", null);
            data.put("message", e.getMessage());
            return ResponseEntity.internalServerError().body(data);
        }

    }

    private Long fibonacci(Long n) throws Exception {
        if (n <= 1)
            return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

}
