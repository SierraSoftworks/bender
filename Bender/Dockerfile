FROM mcr.microsoft.com/dotnet/core/aspnet:3.0-stretch-slim AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/core/sdk:3.0-stretch AS build
WORKDIR /src
COPY ["Bender/Bender.csproj", "Bender/"]
COPY ["Bender.Models/Bender.Models.csproj", "Bender.Models/"]
RUN dotnet restore "Bender/Bender.csproj"
COPY . .
WORKDIR "/src/Bender"
RUN dotnet build "Bender.csproj" -c Release -o /app

FROM build AS publish
RUN dotnet publish "Bender.csproj" -c Release -o /app

FROM base AS final
WORKDIR /app
COPY --from=publish /app .
ENTRYPOINT ["dotnet", "Bender.dll"]