from django.shortcuts import render
from .models import Net
from django.contrib.auth.models import User

from rest_framework import viewsets
from rest_framework.response import Response

from .serializers import NetSerializer, UserSerializer

import subNet.calculations as calculations

import subNet.messages as messages

from rest_framework.authtoken.views import ObtainAuthToken
from rest_framework.authtoken.models import Token
from rest_framework.response import Response

from rest_framework.permissions import AllowAny

class ProfileViewSet(viewsets.ViewSet):
    def create(self, request, pk=0):
        user_id = User.objects.get(pk=request.data.get("user_id"))
        user = User.objects.get(auth_token=request.COOKIES.get('auth_token'))
        if user_id == user.id:
            queryset = User.objects.get(pk=pk)
            serializer = UserSerializer(queryset)
            return Response(serializer.data)

class UserViewSet(viewsets.ViewSet):
    def retrieve(self, request, pk=0):
        queryset = User.objects.get(pk=pk)
        serializer = UserSerializer(queryset)
        return Response(serializer.data)

class ConfirmAuth(viewsets.ViewSet):
    def create(self, request):
        # Check if 'auth_token' is in the request cookies.
        # Give precedence to 'Authorization' header.
        if 'auth_token' in request.COOKIES and \
                        'HTTP_AUTHORIZATION' not in request.META:
            user = User.objects.get(auth_token=request.COOKIES.get('auth_token'))
            return Response({'log': user.id})
        else:
            return Response({'log': 'not logged in'})

            user = User.objects.get(auth_token=request.COOKIES.get('auth_token'))
            following_user = User.objects.get(pk=serializer.data.get("logged_in_user"))

class CustomAuthToken(ObtainAuthToken):
    def post(self, request, *args, **kwargs):
        serializer=self.serializer_class(data=request.data, context={'request':request})
        serializer.is_valid(raise_exception=True)
        user=serializer.validated_data['user']
        token,created=Token.objects.get_or_create(user=user)
        response =  Response({
            "user_id": user.id,
        })
        response.set_cookie(key="auth_token",
                            value=f"{token}",
                            httponly=True,
                            max_age=3600,
                            samesite='None',
                            secure=True,
                            )

        return response
