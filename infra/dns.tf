data "cloudflare_zones" "production" {
  filter {
    account_id = var.cloudflare_account_id
    name       = var.domain
  }
}

resource "azurerm_dns_cname_record" "production" {
  name                = var.name
  resource_group_name = "dns"
  zone_name           = var.domain
  ttl                 = 300
  record              = azurerm_linux_function_app.production.default_hostname
}

resource "azurerm_dns_txt_record" "production-validation" {
  name                = "asuid.${var.name}"
  resource_group_name = "dns"
  zone_name           = var.domain
  ttl                 = 300

  record {
    value = azurerm_linux_function_app.production.custom_domain_verification_id
  }
}

resource "cloudflare_record" "production" {
  name    = var.name
  zone_id = data.cloudflare_zones.production.zones[0].id
  type    = "CNAME"
  value   = azurerm_linux_function_app.production.default_hostname
  ttl     = 300
}

resource "cloudflare_record" "production-validation" {
  name    = "asuid.${var.name}"
  zone_id = data.cloudflare_zones.production.zones[0].id
  type    = "TXT"
  value   = azurerm_linux_function_app.production.custom_domain_verification_id
  ttl     = 300
}

resource "azurerm_dns_cname_record" "staging" {
  name                = "${var.name}-staging"
  resource_group_name = "dns"
  zone_name           = var.domain
  ttl                 = 300
  record              = azurerm_linux_function_app.staging.default_hostname
}

resource "azurerm_dns_txt_record" "staging-validation" {
  name                = "asuid.${var.name}-staging"
  resource_group_name = "dns"
  zone_name           = var.domain
  ttl                 = 300

  record {
    value = azurerm_linux_function_app.staging.custom_domain_verification_id
  }
}

resource "cloudflare_record" "staging" {
  name    = "${var.name}-staging"
  zone_id = data.cloudflare_zones.production.zones[0].id
  type    = "CNAME"
  value   = azurerm_linux_function_app.staging.default_hostname
  ttl     = 300
}

resource "cloudflare_record" "staging-validation" {
  name    = "asuid.${var.name}-staging"
  zone_id = data.cloudflare_zones.production.zones[0].id
  type    = "TXT"
  value   = azurerm_linux_function_app.staging.custom_domain_verification_id
  ttl     = 300
}
