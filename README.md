# Esta es una aplicación de envío gratis de cupón
Esto permite realizar un envío gratis cuando el usuario selecciona el tipo de Courier necesario

## GraphiQL del Admin API 
Es la base de datos del proyecto, dentro de ella se pueden realizar consultas en formato JSON
Para realizar la prueba, se ejecutó el siquiente GraphiQL
-> Esta mutación crea un descuento automático manejado por una app/Function y requiere un scope

mutation CreateAutomaticDiscount {
  discountAutomaticAppCreate(
    automaticAppDiscount: {
      title: "Envío gratis - acordar con comprador"
      functionHandle: "buyer-agreement-shipping"
      discountClasses: [SHIPPING]
      startsAt: "2026-03-23T00:00:00Z"
    }
  ) {
    automaticAppDiscount {
      discountId
      title
      status
    }
    userErrors {
      field
      message
    }
  }
}