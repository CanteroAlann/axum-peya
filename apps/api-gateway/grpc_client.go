package main

import (
	"context"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "axum-peya/api-gateway/proto/tracker"
)

func client() {
	conn, err := grpc.NewClient("restaurant_app:3000", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("Error al conectar: %v", err)
	}
	defer conn.Close()

	client := pb.NewTrackingServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	res, err := client.AddRestaurantLocation(ctx, &pb.RestaurantLocationRequest{Id: 123, Name: "Restaurante Ejemplo", Latitude: 40.7128, Longitude: -74.0060})
	if err != nil {
		log.Fatalf("Error en gRPC: %v", err)
	}

	log.Println("Respuesta gRPC:", res.GetMessage())
}
