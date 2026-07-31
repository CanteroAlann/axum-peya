package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "axum-peya/api-gateway/proto/restaurant"
)

type GatewayServer struct {
	trackerClient pb.RestaurantServiceClient
}

func start_server() {
	conn, err := grpc.NewClient(
		"restaurant_app:3000",
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		log.Fatalf("Error al conectar con el servidor gRPC: %v", err)
	}
	defer conn.Close()

	server := &GatewayServer{
		trackerClient: pb.NewRestaurantServiceClient(conn),
	}

	mux := http.NewServeMux()
	mux.HandleFunc("POST /api/v1/restaurant", server.handleNewRestaurant)

	httpServer := &http.Server{
		Addr:         ":8080",
		Handler:      mux,
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
		IdleTimeout:  120 * time.Second,
	}

	go func() {
		log.Println("API Gateway escuchando en :8080")
		if err := httpServer.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Error crítico en servidor HTTP: %v", err)
		}
	}()

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)
	<-stop

	log.Println("Iniciando apagado limpio (Graceful Shutdown)...")
	ctxShutdown, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := httpServer.Shutdown(ctxShutdown); err != nil {
		log.Fatalf("Error durante el apagado del servidor: %v", err)
	}
	log.Println("Servidor detenido correctamente.")
}

func (s *GatewayServer) handleNewRestaurant(w http.ResponseWriter, r *http.Request) {

	ctx, cancel := context.WithTimeout(r.Context(), 2*time.Second)
	defer cancel()

	var body struct {
		Name      string  `json:"name"`
		Latitude  float64 `json:"latitude"`
		Longitude float64 `json:"longitude"`
	}
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		http.Error(w, "JSON inválido", http.StatusBadRequest)
		return
	}

	res, err := s.trackerClient.NewRestaurant(ctx, &pb.RestaurantRequest{Name: body.Name, Latitude: body.Latitude, Longitude: body.Longitude})
	if err != nil {
		http.Error(w, "Error de comunicación con el microservicio: "+err.Error(), http.StatusBadGateway)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{
		"reply": res.GetMessage(),
	})
}
