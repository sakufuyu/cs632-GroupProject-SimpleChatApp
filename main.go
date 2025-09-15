// main.go
// Simple Text-Based Chat (demo-friendly for non-interactive environments like onecompiler.com)
// If stdin is interactive, it runs the CLI loop (type commands).
// If stdin is non-interactive (e.g. online compilers), it runs a short automated demo and exits.

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
	"time"
)

// Message represents a single chat message.
type Message struct {
	ID        int
	UserID    string
	Text      string
	Timestamp time.Time
}

// MessageStore holds chat history with thread-safety.
type MessageStore struct {
	mu       sync.RWMutex
	messages []Message
	nextID   int
}

func NewMessageStore() *MessageStore {
	return &MessageStore{messages: make([]Message, 0), nextID: 1}
}

func (s *MessageStore) Add(userID, text string) Message {
	s.mu.Lock()
	defer s.mu.Unlock()
	msg := Message{ID: s.nextID, UserID: userID, Text: text, Timestamp: time.Now()}
	s.messages = append(s.messages, msg)
	s.nextID++
	return msg
}

func (s *MessageStore) All() []Message {
	s.mu.RLock()
	defer s.mu.RUnlock()
	cpy := make([]Message, len(s.messages))
	copy(cpy, s.messages)
	return cpy
}

func (s *MessageStore) FilterByUser(userID string) []Message {
	s.mu.RLock()
	defer s.mu.RUnlock()
	var res []Message
	for _, m := range s.messages {
		if strings.EqualFold(m.UserID, userID) {
			res = append(res, m)
		}
	}
	return res
}

func (s *MessageStore) SearchByKeyword(keyword string) []Message {
	s.mu.RLock()
	defer s.mu.RUnlock()
	kw := strings.ToLower(keyword)
	var res []Message
	for _, m := range s.messages {
		if strings.Contains(strings.ToLower(m.Text), kw) {
			res = append(res, m)
		}
	}
	return res
}

// Dispatcher receives MessageInput, stores, and broadcasts.
type Dispatcher struct {
	store    *MessageStore
	incoming chan MessageInput
	quit     chan struct{}
}

type MessageInput struct {
	UserID string
	Text   string
}

func NewDispatcher(store *MessageStore) *Dispatcher {
	return &Dispatcher{
		store:    store,
		incoming: make(chan MessageInput, 100),
		quit:     make(chan struct{}),
	}
}

func (d *Dispatcher) Start() {
	go func() {
		for {
			select {
			case mi := <-d.incoming:
				msg := d.store.Add(mi.UserID, mi.Text)
				d.broadcast(msg)
			case <-d.quit:
				return
			}
		}
	}()
}

func (d *Dispatcher) Stop() {
	// safe to close even if already closed by defer in main
	select {
	case <-d.quit:
		// already closed
	default:
		close(d.quit)
	}
}

func (d *Dispatcher) Send(userID, text string) {
	d.incoming <- MessageInput{UserID: userID, Text: text}
}

func (d *Dispatcher) broadcast(msg Message) {
	// Print broadcast line that simulates delivering to connected clients.
	fmt.Printf("[%s] %s: %s\n", msg.Timestamp.Format("2006-01-02 15:04:05"), msg.UserID, msg.Text)
}

// simulateUser periodically sends messages from a simulated user.
func simulateUser(name string, d *Dispatcher, messages []string, interval time.Duration, stopCh <-chan struct{}) {
	go func() {
		idx := 0
		ticker := time.NewTicker(interval)
		defer ticker.Stop()
		for {
			select {
			case <-ticker.C:
				d.Send(name, messages[idx%len(messages)])
				idx++
			case <-stopCh:
				return
			}
		}
	}()
}

func prettyPrintMessages(msgs []Message) {
	if len(msgs) == 0 {
		fmt.Println("No messages found.")
		return
	}
	for _, m := range msgs {
		fmt.Printf("#%d [%s] %s: %s\n", m.ID, m.Timestamp.Format("2006-01-02 15:04:05"), m.UserID, m.Text)
	}
}

func main() {
	fmt.Println("=== Simple Text-Based Chat (Go) ===")
	fmt.Println("Type 'help' for commands (in interactive terminals).")

	store := NewMessageStore()
	dispatcher := NewDispatcher(store)
	dispatcher.Start()
	defer dispatcher.Stop()

	// Simulated users and a stop channel for them.
	simStop := make(chan struct{})
	defer close(simStop)
	simulatedUsers := map[string][]string{
		"Alice": {"Hello!", "Anyone up for coffee?", "I am debugging Go code."},
		"Bob":   {"Hey all", "I pushed a change to the repo", "Will test now"},
		"Eve":   {"Good morning :)", "Reminder: meeting at 3pm", "Nice work team!"},
	}
	simulateUser("Alice", dispatcher, simulatedUsers["Alice"], 3*time.Second, simStop)
	simulateUser("Bob", dispatcher, simulatedUsers["Bob"], 5*time.Second, simStop)
	// Eve left simulated to show fewer msgs; still we can send Eve messages manually in demo.

	// Seed a system welcome message
	dispatcher.Send("System", "Welcome to the chat. Simulated users: Alice, Bob, Eve.")

	// Detect if stdin is interactive. Online compilers are typically non-interactive.
	fi, _ := os.Stdin.Stat()
	nonInteractive := (fi.Mode() & os.ModeCharDevice) == 0

	if nonInteractive {
		// Demo mode for non-interactive environment (like onecompiler)
		fmt.Println("\n[demo mode] Non-interactive environment detected. Running automated demo...\n")
		// Let simulated users generate a few messages
		time.Sleep(1200 * time.Millisecond)
		dispatcher.Send("Eve", "I'll join the meeting in 10 mins.")
		time.Sleep(800 * time.Millisecond)
		dispatcher.Send("Tester", "This is a demo from Tester.")
		// Wait a bit so background simulated users print some lines
		time.Sleep(3200 * time.Millisecond)

		fmt.Println("\n--- Full history ---")
		prettyPrintMessages(store.All())

		fmt.Println("\n--- Messages by user: Alice ---")
		prettyPrintMessages(store.FilterByUser("Alice"))

		fmt.Println("\n--- Search keyword: meeting ---")
		prettyPrintMessages(store.SearchByKeyword("meeting"))

		fmt.Println("\n[demo mode] Demo complete. Exiting.")
		return
	}

	// Interactive CLI loop (for real terminals only)
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("> ")
		if !scanner.Scan() {
			if err := scanner.Err(); err != nil {
				fmt.Fprintf(os.Stderr, "\n[error] scanner error: %v\n", err)
			} else {
				fmt.Fprintln(os.Stderr, "\n[info] input closed (EOF). Exiting.")
			}
			break
		}
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		cmd := strings.ToLower(parts[0])

		switch cmd {
		case "quit", "exit":
			fmt.Println("Exiting chat. Bye!")
			return
		case "help":
			fmt.Println("Commands:")
			fmt.Println("  send <UserID> <message...>  - send a message as UserID")
			fmt.Println("  history                     - show all messages")
			fmt.Println("  search user <UserID>        - show messages by a user")
			fmt.Println("  search keyword <word>       - search messages by keyword")
			fmt.Println("  help                        - show this help")
			fmt.Println("  quit / exit                 - exit the program")
		case "send":
			if len(parts) < 3 {
				fmt.Println("Usage: send <UserID> <message...>")
				continue
			}
			userID := parts[1]
			text := strings.Join(parts[2:], " ")
			dispatcher.Send(userID, text)
		case "history":
			prettyPrintMessages(store.All())
		case "search":
			if len(parts) < 3 {
				fmt.Println("Usage: search user <UserID>  OR  search keyword <word>")
				continue
			}
			subcmd := strings.ToLower(parts[1])
			arg := strings.Join(parts[2:], " ")
			if subcmd == "user" {
				prettyPrintMessages(store.FilterByUser(arg))
			} else if subcmd == "keyword" {
				prettyPrintMessages(store.SearchByKeyword(arg))
			} else {
				fmt.Println("Unknown search subcommand. Use: user OR keyword")
			}
		default:
			fmt.Println("Unknown command. Type 'help' for commands.")
		}
	}
}