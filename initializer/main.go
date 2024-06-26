package main

import (
	"database/sql"
	"fmt"
	"os"

	_ "github.com/trinodb/trino-go-client/trino"
)

func main() {
	// Get user, password, catalog from env vars TRINO_*
	// from env var TRINO_USER or "trino"
	user := os.Getenv("TRINO_USER")
	if user == "" {
		user = "admin"
	}
	// from env var TRINO_PASSWORD or ""
	password := os.Getenv("TRINO_PASSWORD")
	if password == "" {
		password = ""
	}
	// from env var TRINO_HOST or "trion"
	host := os.Getenv("TRINO_HOST")
	if host == "" {
		host = "localhost"
	}
	// from env var TRINO_CATALOG or "hive"
	catalog := os.Getenv("TRINO_CATALOG")
	if catalog == "" {
		catalog = "iceberg"
	}

	// Combine the values into a DSN string
	dsn := fmt.Sprintf("http://%s:%s@%s:8080/%s", user, password, host, catalog)
	fmt.Println(fmt.Sprintf("Connecting to %s", dsn))
	db, err := sql.Open("trino", dsn)
	if err != nil {
		panic(err)
	}

	// First command
	user_result, err := db.Query("SELECT current_user")
	if err != nil {
		panic(err)
	} else {
		for user_result.Next() {
			var user string
			err := user_result.Scan(&user)
			if err != nil {
				panic(err)
			}
			msg := fmt.Sprintf("Current user: %s", user)
			fmt.Println(msg)
		}
	}

	// Create schema
	create_result, err := db.Exec("CREATE SCHEMA IF NOT EXISTS iceberg.phoenix")
	if err != nil {
		panic(err)
	} else {
		fmt.Println("Done creating schema iceberg.phoenix")
		_ = create_result
	}

	// Create table: Transaction
	txn_query := `CREATE TABLE IF NOT EXISTS iceberg.phoenix.transaction (
		signature varchar,
		timestamp timestamp,
		successful boolean,
		confirmation_status varchar,
		slot bigint,
		fee bigint,
		compute_units bigint
	)`
	txn_result, err := db.Exec(txn_query)
	if err != nil {
		panic(err)
	} else {
		fmt.Println("Done creating table iceberg.phoenix.transaction")
		_ = txn_result
	}
	// Create table: Event
	evt_query := `CREATE TABLE IF NOT EXISTS iceberg.phoenix.event (
		id bigint, 
		name varchar
	)`
	evt_result, err := db.Exec(evt_query)
	if err != nil {
		panic(err)
	} else {
		fmt.Println("Done creating table iceberg.phoenix.event")
		_ = evt_result
	}
}
