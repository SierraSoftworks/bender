resource "azurerm_service_plan" "function" {
  name                = "${var.name}-serviceplan"
  location            = azurerm_resource_group.function.location
  resource_group_name = azurerm_resource_group.function.name
  os_type             = "Linux"
  sku_name            = "Y1"
}

data "azurerm_linux_function_app" "production" {
  name                = "${var.name}-sierrasoftworks"
  resource_group_name = azurerm_resource_group.function.name
}

resource "azurerm_linux_function_app" "production" {
  name                 = "${var.name}-sierrasoftworks"
  location             = azurerm_resource_group.function.location
  resource_group_name  = azurerm_resource_group.function.name
  service_plan_id      = azurerm_service_plan.function.id
  storage_account_name = azurerm_storage_account.function.name
  //storage_account_access_key    = azurerm_storage_account.function.primary_access_key
  storage_uses_managed_identity = true

  https_only = true

  site_config {
    http2_enabled      = true
    websockets_enabled = true

    cors {
      allowed_origins = var.allowed_origins
    }

    application_stack {
      use_custom_runtime = true
    }
  }

  identity {
    type = "SystemAssigned"
  }

  app_settings = merge(
    data.azurerm_linux_function_app.production.app_settings,
    {
      "OTEL_EXPORTER_OTLP_ENDPOINT" = "https://refinery.sierrasoftworks.com",
      "OTEL_EXPORTER_OTLP_HEADERS"  = "x-honeycomb-team=${var.honeycomb-key-production}",
      "OTEL_EXPORTER_OTLP_PROTOCOL" = "http-binary",
      "OTEL_SERVICE_NAME"           = "${var.name}",
      "HONEYCOMB_KEY"               = "${var.honeycomb-key-production}",
      "HONEYCOMB_DATASET"           = "${honeycombio_dataset.dataset.name}",
    },
    var.app-settings
  )
}

resource "azurerm_app_service_custom_hostname_binding" "production" {
  hostname            = "${var.name}.${var.domain}"
  app_service_name    = azurerm_linux_function_app.production.name
  resource_group_name = azurerm_resource_group.function.name

  lifecycle {
    ignore_changes = [ssl_state, thumbprint]
  }

  depends_on = [
    azurerm_dns_txt_record.production-validation
  ]
}

resource "azurerm_app_service_managed_certificate" "production" {
  custom_hostname_binding_id = azurerm_app_service_custom_hostname_binding.production.id
}

resource "azurerm_app_service_certificate_binding" "production" {
  hostname_binding_id = azurerm_app_service_custom_hostname_binding.production.id
  certificate_id      = azurerm_app_service_managed_certificate.production.id
  ssl_state           = "SniEnabled"
}


data "azurerm_linux_function_app" "staging" {
  name                = "${var.name}-sierrasoftworks-staging"
  resource_group_name = azurerm_resource_group.function.name
}

resource "azurerm_linux_function_app" "staging" {
  name                 = "${var.name}-sierrasoftworks-staging"
  location             = azurerm_resource_group.function.location
  resource_group_name  = azurerm_resource_group.function.name
  service_plan_id      = azurerm_service_plan.function.id
  storage_account_name = azurerm_storage_account.function.name
  // storage_account_access_key    = azurerm_storage_account.function.primary_access_key
  storage_uses_managed_identity = true

  https_only = true

  site_config {
    http2_enabled      = true
    websockets_enabled = true

    cors {
      allowed_origins = var.allowed_origins
    }

    application_stack {
      use_custom_runtime = true
    }
  }

  identity {
    type = "SystemAssigned"
  }

  app_settings = merge(
    data.azurerm_linux_function_app.staging.app_settings,
    {
      "OTEL_EXPORTER_OTLP_ENDPOINT" = "https://api.honeycomb.io",
      "OTEL_EXPORTER_OTLP_HEADERS"  = "x-honeycomb-team=${var.honeycomb-key-staging}",
      "OTEL_SERVICE_NAME"           = "${var.name}"
      "HONEYCOMB_KEY"               = "${var.honeycomb-key-staging}",
      "HONEYCOMB_DATASET"           = "${honeycombio_dataset.dataset.name}",
    },
    var.app-settings
  )
}

resource "azurerm_app_service_custom_hostname_binding" "staging" {
  hostname            = "${var.name}-staging.${var.domain}"
  app_service_name    = azurerm_linux_function_app.staging.name
  resource_group_name = azurerm_resource_group.function.name

  lifecycle {
    ignore_changes = [ssl_state, thumbprint]
  }

  depends_on = [
    azurerm_dns_txt_record.staging-validation
  ]
}

resource "azurerm_app_service_managed_certificate" "staging" {
  custom_hostname_binding_id = azurerm_app_service_custom_hostname_binding.staging.id
}

resource "azurerm_app_service_certificate_binding" "staging" {
  hostname_binding_id = azurerm_app_service_custom_hostname_binding.staging.id
  certificate_id      = azurerm_app_service_managed_certificate.staging.id
  ssl_state           = "SniEnabled"
}
